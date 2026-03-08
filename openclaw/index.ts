declare function require(moduleName: "child_process"): {
  spawnSync: (
    command: string,
    args: string[],
    options: { encoding: string },
  ) => {
    stdout?: string;
    error?: { message: string };
  };
};

const { spawnSync } = require("child_process");

type RewriteDecision = {
  original_command: string;
  rewritten_command?: string | null;
  matched_rule?: string | null;
  skip_reason?: string | null;
  excluded_by_config: boolean;
};

type ToolContext = {
  agentId?: string;
  sessionKey?: string;
};

type BeforeToolCallEvent = {
  toolName: string;
  params: Record<string, unknown>;
};

type BeforeToolCallResult = {
  block?: boolean;
  blockReason?: string;
  params?: Record<string, unknown>;
};

type PluginConfig = {
  enabled?: boolean;
  verbose?: boolean;
  rtkPath?: string;
};

type PluginApi = {
  config?: PluginConfig;
  on: (
    hookName: string,
    handler: (event: BeforeToolCallEvent, context: ToolContext) => BeforeToolCallResult | void,
    options?: { priority?: number },
  ) => void;
  logger?: {
    info?: (message: string) => void;
    warn?: (message: string) => void;
  };
};

function log(api: PluginApi, level: "info" | "warn", message: string) {
  api.logger?.[level]?.(message);
}

function buildTrackingEnv(context: ToolContext, decision: RewriteDecision): Record<string, string> {
  const env: Record<string, string> = {
    RTK_HOST: "openclaw",
    RTK_TOOL_NAME: "exec",
  };

  if (context.sessionKey) {
    env.RTK_SESSION_KEY = context.sessionKey;
  }

  if (context.agentId) {
    env.RTK_AGENT_ID = context.agentId;
  }

  if (decision.matched_rule) {
    env.RTK_MATCHED_RULE = decision.matched_rule;
  }

  return env;
}

function buildEnvPrefix(env: Record<string, string>): string {
  const parts: string[] = [];
  for (const key in env) {
    parts.push(`${key}=${JSON.stringify(env[key])}`);
  }
  return `${parts.join(" ")} `;
}

function rewriteCommand(api: PluginApi, command: string): RewriteDecision | null {
  const rtkPath = api.config?.rtkPath || "rtk";
  const result = spawnSync(rtkPath, ["rewrite", "--json", command], {
    encoding: "utf8",
  });

  if (!result.stdout) {
    if (result.error) {
      log(api, "warn", `[rtk-openclaw] rewrite failed: ${result.error.message}`);
    }
    return null;
  }

  try {
    return JSON.parse(result.stdout) as RewriteDecision;
  } catch {
    log(api, "warn", "[rtk-openclaw] failed to parse rewrite JSON");
    return null;
  }
}

export default function register(api: PluginApi) {
  if (api.config?.enabled === false) {
    return;
  }

  api.on(
    "before_tool_call",
    (event, context) => {
      if (event.toolName !== "exec") {
        return;
      }

      const command = event.params.command;
      if (typeof command !== "string" || command.trim().length === 0) {
        return;
      }

      const decision = rewriteCommand(api, command);
      if (!decision?.rewritten_command) {
        if (api.config?.verbose && decision?.skip_reason) {
          log(api, "info", `[rtk-openclaw] skip ${command}: ${decision.skip_reason}`);
        }
        return;
      }

      const trackingEnv = buildTrackingEnv(context, decision);
      const existingEnv = event.params.env;
      const mergedEnv =
        existingEnv && typeof existingEnv === "object"
          ? { ...(existingEnv as Record<string, unknown>), ...trackingEnv }
          : trackingEnv;
      const rewritten =
        existingEnv === undefined || (existingEnv && typeof existingEnv === "object")
          ? decision.rewritten_command
          : `${buildEnvPrefix(trackingEnv)}${decision.rewritten_command}`;

      if (api.config?.verbose) {
        log(api, "info", `[rtk-openclaw] ${command} -> ${rewritten}`);
      }

      return {
        params: {
          ...event.params,
          command: rewritten,
          env: mergedEnv,
        },
      };
    },
    { priority: 100 },
  );
}
