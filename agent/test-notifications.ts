/**
 * Integration Test: Proactive Notification Flow
 * Demonstrates the complete notification system working end-to-end
 */

import { projectGoalStatus } from './goal-projection.js';
import { monitorUserGoals, type UserGoal } from './notification-service.js';
import { generateProactiveMessage } from './agent-service.js';

/**
 * Simulate a user falling behind on their goal
 */
async function testProactiveNotification() {
  console.log('🧪 Testing Proactive Notification Flow...\n');

  // Scenario: User's balance drops due to market downturn
  const userGoal: UserGoal = {
    userId: 'test-user-001',
    currentBalance: 10000, // Down from 12450 (market downturn)
    targetAmount: 18000,
    targetDate: new Date('2026-08-01'),
    expectedAPY: 8.5,
    monthlyContribution: 500,
    hasNotified: false,
  };

  console.log('📊 User Goal:');
  console.log(`   Current Balance: $${userGoal.currentBalance}`);
  console.log(`   Target: $${userGoal.targetAmount}`);
  console.log(`   Target Date: ${userGoal.targetDate.toLocaleDateString()}`);
  console.log(`   Monthly Contribution: $${userGoal.monthlyContribution}\n`);

  // Check goal status
  const projection = projectGoalStatus(
    userGoal.currentBalance,
    userGoal.targetAmount,
    userGoal.targetDate,
    userGoal.expectedAPY,
    userGoal.monthlyContribution
  );

  console.log('📈 Projection Results:');
  console.log(`   Status: ${projection.status}`);
  console.log(`   Projected Value: $${projection.projectedValue.toFixed(2)}`);
  console.log(`   Shortfall: $${projection.shortfall.toFixed(2)}`);
  console.log(`   Required Monthly: $${projection.requiredMonthlyContribution.toFixed(2)}`);
  console.log(
    `   Suggested Increase: $${(
      projection.requiredMonthlyContribution - userGoal.monthlyContribution
    ).toFixed(2)}\n`
  );

  // Generate proactive message
  if (projection.status === 'Falling Behind') {
    console.log('🤖 Generating OpenClaw Message...\n');

    const apiKey = process.env.ANTHROPIC_API_KEY;
    if (!apiKey) {
      console.log('⚠️  ANTHROPIC_API_KEY not set. Using fallback message.\n');
    }

    try {
      const message = await generateProactiveMessage(
        {
          userId: userGoal.userId,
          goalName: 'European Vacation',
          currentBalance: userGoal.currentBalance,
          targetAmount: userGoal.targetAmount,
          targetDate: userGoal.targetDate.toLocaleDateString(),
          projectedValue: projection.projectedValue,
          shortfall: projection.shortfall,
          monthlyContribution: userGoal.monthlyContribution,
          requiredIncrease: Math.max(
            0,
            projection.requiredMonthlyContribution - userGoal.monthlyContribution
          ),
        },
        apiKey || ''
      );

      console.log('✉️  Notification Message:');
      console.log(`   "${message}"\n`);
    } catch (error) {
      console.error('❌ Error generating message:', error);
    }

    console.log(
      '✅ Notification would be sent to user via WebSocket\n'
    );
  } else {
    console.log('✓ User is on track, no notification needed.\n');
  }

  console.log('🧪 Test complete!\n');
}

// Run test if this file is executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
  testProactiveNotification().catch(console.error);
}
