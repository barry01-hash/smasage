/**
 * OpenClaw Agent Service - Proactive Message Generation
 * Uses Claude LLM to generate contextual, empathetic messages and solutions
 */

export interface GoalContext {
  userId: string;
  goalName: string;
  currentBalance: number;
  targetAmount: number;
  targetDate: string;
  projectedValue: number;
  shortfall: number;
  monthlyContribution: number;
  requiredIncrease: number;
}

const AGENT_PROMPT = `You are Smasage OpenClaw, a compassionate and witty financial advisor on the Stellar network.

Your job is to generate SHORT, genuinely helpful proactive notifications that nudge users when their savings goals are at risk.

RULES:
1. Keep messages to 1-2 sentences max (this is a notification, not an essay)
2. Be conversational and encouraging, not preachy
3. Always lead with empathy ("Hey, I noticed..." or "Looks like...")
4. Suggest concrete, specific actions (exact dollar amounts)
5. Use casual language with appropriate emojis
6. Never be accusatory—frame as "we" not "you"
7. Include a clear call-to-action at the end

TONE: Friendly, supportive, like a trusted friend who understands finance`;

/**
 * Generate proactive message using Claude
 */
export async function generateProactiveMessage(
  context: GoalContext,
  apiKey: string
): Promise<string> {
  const userPrompt = `
Generate a proactive notification for this user:

Goal: ${context.goalName}
Current Balance: $${context.currentBalance.toFixed(2)}
Target: $${context.targetAmount.toFixed(2)} by ${context.targetDate}
Projected Value: $${context.projectedValue.toFixed(2)}
Shortfall: $${context.shortfall.toFixed(2)}
Current Monthly Contribution: $${context.monthlyContribution}
Suggested Increase: $${context.requiredIncrease}

Generate a notification message that:
1. Explains the situation empathetically in 1 sentence
2. Proposes the exact solution in 1 sentence
3. Ends with "Ready to adjust? New monthly: $${context.monthlyContribution + context.requiredIncrease}. Sound good?"

Keep it short, conversational, and actionable.`;

  try {
    const response = await fetch('https://api.anthropic.com/v1/messages', {
      method: 'POST',
      headers: {
        'x-api-key': apiKey,
        'anthropic-version': '2023-06-01',
        'content-type': 'application/json',
      },
      body: JSON.stringify({
        model: 'claude-3-5-sonnet-20241022',
        max_tokens: 256,
        system: AGENT_PROMPT,
        messages: [
          {
            role: 'user',
            content: userPrompt,
          },
        ],
      }),
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`Claude API error: ${response.status} — ${error}`);
    }

    const data = await response.json();
    return data.content[0].text;
  } catch (error) {
    console.error('Error generating proactive message:', error);
    // Fallback to template if API fails
    return generateFallbackMessage(context);
  }
}

/**
 * Generate fallback message when API is unavailable
 */
function generateFallbackMessage(context: GoalContext): string {
  const increase = context.requiredIncrease;
  const newTotal = context.monthlyContribution + increase;

  if (increase > 0) {
    return `📊 Hey! Your ${context.goalName} goal is on the radar. If we bump your monthly from $${context.monthlyContribution} to $${newTotal}, you'll crush the $${context.targetAmount} target by ${context.targetDate}. Let's make it happen?`;
  } else {
    return `✨ Your ${context.goalName} goal looking good! Current pace will get you to $${context.targetAmount} by ${context.targetDate}. Keep up the momentum!`;
  }
}

/**
 * Generate suggestion text for rebalancing portfolio
 */
export function generateAllocationSuggestion(
  shortfallPercent: number
): string {
  if (shortfallPercent > 15) {
    return "I'd suggest shifting to a more aggressive portfolio mix—maybe 40% Blend, 50% Soroswap LP, 10% Gold—to boost returns.";
  } else if (shortfallPercent > 7) {
    return "Consider a balanced approach: 50% Blend, 40% Soroswap LP, 10% Gold to optimize your returns.";
  } else {
    return "Your current allocation is solid. A small boost to monthly contributions should get you back on track!";
  }
}

/**
 * Parse and validate suggested adjustment
 */
export interface SuggestedAdjustment {
  newMonthlyContribution: number;
  allocationShift?: {
    blend: number;
    soroswap: number;
    gold: number;
  };
}

export function extractSuggestion(message: string): SuggestedAdjustment | null {
  // Look for currency amounts in the message
  const amounts = message.match(/\$(\d+)/g);
  if (amounts && amounts.length > 0) {
    const newContribution = parseInt(amounts[amounts.length - 1].replace('$', ''));
    return {
      newMonthlyContribution: newContribution,
    };
  }

  return null;
}
