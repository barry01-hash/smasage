/**
 * Proactive Notification Service
 * Monitors user goals and triggers pre-emptive messages when "Falling Behind"
 * Works in conjunction with the WebSocket server to push notifications to clients
 */

import { projectGoalStatus } from './goal-projection.js';

export interface UserGoal {
  userId: string;
  currentBalance: number;
  targetAmount: number;
  targetDate: Date;
  expectedAPY: number;
  monthlyContribution: number;
  hasNotified?: boolean; // Track if already notified for this status
}

export interface ProactiveNotification {
  type: 'falling-behind' | 'ahead' | 'on-track';
  userId: string;
  timestamp: string;
  message: string;
  suggestion?: string;
  projection: {
    status: string;
    projectedValue: number;
    shortfall?: number;
    surplus?: number;
    requiredMonthlyContribution?: number;
  };
}

/**
 * Check if a user's goal status warrants a proactive notification
 */
export function checkGoalStatus(goal: UserGoal): ProactiveNotification | null {
  const projection = projectGoalStatus(
    goal.currentBalance,
    goal.targetAmount,
    goal.targetDate,
    goal.expectedAPY,
    goal.monthlyContribution
  );

  // Only notify if status changed to "Falling Behind"
  if (projection.status === 'Falling Behind') {
    const notification = generateProactiveMessage(
      goal.userId,
      projection,
      goal
    );
    return notification;
  }

  return null;
}

/**
 * Generate proactive message with AI-suggested solution
 */
function generateProactiveMessage(
  userId: string,
  projection: ReturnType<typeof projectGoalStatus>,
  goal: UserGoal
): ProactiveNotification {
  const shortfallPercent = (
    (projection.shortfall / goal.targetAmount) * 100
  ).toFixed(1);

  // Calculate suggested increase in monthly contribution
  const suggestedIncrease = Math.ceil(projection.requiredMonthlyContribution - goal.monthlyContribution);
  const newMonthlyTotal = goal.monthlyContribution + suggestedIncrease;

  // Generate natural-sounding message
  const message = `📊 Hey! I noticed your savings projection has shifted. With your current pace, you're tracking to fall short by $${projection.shortfall.toFixed(
    2
  )} (${shortfallPercent}% below target). But don't worry—we can fix this!`;

  const suggestion =
    suggestedIncrease > 0
      ? `If you increase your monthly deposit from $${goal.monthlyContribution} to $${newMonthlyTotal}, you'll get back on track to hit your $${goal.targetAmount.toFixed(2)} goal by ${goal.targetDate.toLocaleDateString()}. Want to give it a try?`
      : `Your current monthly contribution of $${goal.monthlyContribution} might be too conservative. Consider a small boost to accelerate your growth. What do you think?`;

  return {
    type: 'falling-behind',
    userId,
    timestamp: new Date().toISOString(),
    message,
    suggestion,
    projection: {
      status: projection.status,
      projectedValue: projection.projectedValue,
      shortfall: projection.shortfall,
      requiredMonthlyContribution: projection.requiredMonthlyContribution,
    },
  };
}

/**
 * Monitor a set of user goals and return notifications for those who need them
 */
export function monitorUserGoals(goals: UserGoal[]): ProactiveNotification[] {
  const notifications: ProactiveNotification[] = [];

  for (const goal of goals) {
    const notification = checkGoalStatus(goal);
    if (notification) {
      notifications.push(notification);
    }
  }

  return notifications;
}

/**
 * Parse suggested contribution increase from notification
 */
export function parseSuggestedContribution(
  suggestion: string
): number | null {
  const match = suggestion.match(/\$(\d+)/g);
  if (match && match.length >= 2) {
    // Second match is the new total
    return parseInt(match[1].replace('$', ''));
  }
  return null;
}
