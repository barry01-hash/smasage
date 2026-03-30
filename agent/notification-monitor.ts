/**
 * Notification Monitor Service
 * Runs the WebSocket server and monitors user goals for proactive notifications
 */

import { createServer } from 'http';
import { NotificationServer } from './websocket-server.js';
import { projectGoalStatus, type GoalProjection } from './goal-projection.js';
import { type UserGoal } from './notification-service.js';
import { generateProactiveMessage } from './agent-service.js';

const PORT = parseInt(process.env.NOTIFICATION_PORT || '3001', 10);
const API_KEY = process.env.ANTHROPIC_API_KEY;

if (!API_KEY) {
  console.error('ERROR: ANTHROPIC_API_KEY is not set in .env file');
  process.exit(1);
}

// Create HTTP server for WebSocket
const httpServer = createServer();
const notificationServer = new NotificationServer(httpServer);

/**
 * Trigger a proactive message if goal status warrants it
 */
async function triggerProactiveMessage(
  goal: UserGoal,
  projection: GoalProjection
): Promise<void> {
  if (projection.status !== 'Falling Behind') {
    return;
  }

  // Generate AI-powered message
  const message = await generateProactiveMessage(
    {
      userId: goal.userId,
      goalName: 'Financial Goal',
      currentBalance: goal.currentBalance,
      targetAmount: goal.targetAmount,
      targetDate: goal.targetDate.toLocaleDateString(),
      projectedValue: projection.projectedValue,
      shortfall: projection.shortfall,
      monthlyContribution: goal.monthlyContribution,
      requiredIncrease: Math.max(
        0,
        projection.requiredMonthlyContribution - goal.monthlyContribution
      ),
    },
    API_KEY
  );

  // Send notification through WebSocket
  const client = (notificationServer as any).clients?.get(goal.userId);
  if (client) {
    notificationServer.broadcastMessage({
      type: 'agent-message',
      userId: goal.userId,
      payload: {
        sender: 'agent',
        text: message,
        timestamp: new Date().toISOString(),
        proactive: true,
        projection: {
          status: projection.status,
          projectedValue: projection.projectedValue,
          shortfall: projection.shortfall,
          requiredMonthlyContribution: projection.requiredMonthlyContribution,
        },
      },
    });
  }
}

/**
 * Start monitoring service
 */
function startMonitoring(): void {
  httpServer.listen(PORT, () => {
    console.log(`✅ Notification Server listening on ws://localhost:${PORT}`);
  });

  // Graceful shutdown
  process.on('SIGTERM', () => {
    console.log('SIGTERM received, shutting down gracefully...');
    notificationServer.shutdown();
    httpServer.close(() => {
      console.log('Server closed');
      process.exit(0);
    });
  });

  process.on('SIGINT', () => {
    console.log('SIGINT received, shutting down gracefully...');
    notificationServer.shutdown();
    httpServer.close(() => {
      console.log('Server closed');
      process.exit(0);
    });
  });
}

// Start the service
startMonitoring();

console.log(`
╔══════════════════════════════════════╗
║  Smasage Notification Monitor       ║
║  Ready for Proactive Nudges  📊      ║
╚══════════════════════════════════════╝
`);
