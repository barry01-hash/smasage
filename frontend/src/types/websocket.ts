/**
 * WebSocket Payload Types – Issue #45
 *
 * Defines strict interfaces for every WebSocket message payload and provides
 * type guard helpers so callers can safely narrow `notification.payload`
 * without using `as any` or unsafe casts.
 */

// ---------------------------------------------------------------------------
// Outgoing message payloads (client → server)
// ---------------------------------------------------------------------------

/** Shape of a goal object sent to the server when registering or updating. */
export interface GoalPayload {
  currentBalance: number;
  targetAmount: number;
  targetDate: string;
  monthlyContribution: number;
  expectedAPY: number;
}

/** Wrapper for register-goal messages. */
export interface RegisterGoalMessage {
  type: 'register-goal';
  payload: GoalPayload;
}

/** Wrapper for update-goal messages. */
export interface UpdateGoalMessage {
  type: 'update-goal';
  payload: GoalPayload;
}

/** Wrapper for ping messages. */
export interface PingMessage {
  type: 'ping';
  timestamp: string;
}

/** Union of all outgoing message types. */
export type OutgoingWsMessage = RegisterGoalMessage | UpdateGoalMessage | PingMessage;

// ---------------------------------------------------------------------------
// Incoming payload shapes (server → client)
// ---------------------------------------------------------------------------

/** Payload for `agent-message` notifications. */
export interface AgentMessagePayload {
  text: string;
  proactive?: boolean;
  timestamp?: string;
}

/** Payload for `goal-update` notifications (server-pushed goal status). */
export interface GoalUpdatePayload {
  currentBalance: number;
  targetAmount: number;
  progressPercentage: number;
  status: 'on-track' | 'ahead' | 'falling-behind';
}

// ---------------------------------------------------------------------------
// Discriminated union of all incoming notifications
// ---------------------------------------------------------------------------

interface ConnectedNotification {
  type: 'connected';
  userId?: string;
  payload?: undefined;
  timestamp?: string;
}

interface AgentMessageNotification {
  type: 'agent-message';
  userId?: string;
  payload: AgentMessagePayload;
  timestamp?: string;
}

interface GoalUpdateNotification {
  type: 'goal-update';
  userId?: string;
  payload: GoalUpdatePayload;
  timestamp?: string;
}

interface PongNotification {
  type: 'pong';
  userId?: string;
  payload?: undefined;
  timestamp?: string;
}

/** Generic notification – used as the catch-all / unknown type. */
interface GenericNotification {
  type: 'notification';
  userId?: string;
  payload?: unknown;
  timestamp?: string;
}

/**
 * Discriminated union covering every notification type the server can send.
 * Add new variants here as the API evolves.
 */
export type IncomingNotification =
  | ConnectedNotification
  | AgentMessageNotification
  | GoalUpdateNotification
  | PongNotification
  | GenericNotification;

// ---------------------------------------------------------------------------
// Type guards
// ---------------------------------------------------------------------------

/** Narrows a notification to `AgentMessageNotification`. */
export function isAgentMessageNotification(
  n: IncomingNotification
): n is AgentMessageNotification {
  return n.type === 'agent-message';
}

/** Narrows a notification to `GoalUpdateNotification`. */
export function isGoalUpdateNotification(
  n: IncomingNotification
): n is GoalUpdateNotification {
  return n.type === 'goal-update';
}

/** Narrows a notification to the `connected` variant. */
export function isConnectedNotification(
  n: IncomingNotification
): n is ConnectedNotification {
  return n.type === 'connected';
}
