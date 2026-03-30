/**
 * Proactive Suggestion Handler
 * Utilities for processing user acceptance of proactive suggestions
 */

interface ProactiveSuggestion {
  type: 'contribution-increase' | 'allocation-rebalance' | 'both';
  currentContribution: number;
  suggestedContribution?: number;
  currentAllocation?: Record<string, number>;
  suggestedAllocation?: Record<string, number>;
}

/**
 * Parse suggestion from proactive message
 */
export function extractSuggestionFromMessage(
  message: string
): ProactiveSuggestion | null {
  // Look for contribution amounts like "$50" or "$600"
  const amounts = message.match(/\$(\d+)/g);

  if (!amounts || amounts.length < 2) {
    return null;
  }

  // Typically: first amount is current, last is suggested
  const currentAmount = parseInt(amounts[0].replace('$', ''));
  const suggestedAmount = parseInt(amounts[amounts.length - 1].replace('$', ''));

  if (suggestedAmount > currentAmount) {
    return {
      type: 'contribution-increase',
      currentContribution: currentAmount,
      suggestedContribution: suggestedAmount,
    };
  }

  return null;
}

/**
 * Apply proactive suggestion (update user goal)
 */
export function applySuggestion(
  suggestion: ProactiveSuggestion,
  onUpdateGoal: (updates: Record<string, unknown>) => void
): void {
  const updates: Record<string, unknown> = {};

  if (suggestion.suggestedContribution) {
    updates.monthlyContribution = suggestion.suggestedContribution;
  }

  if (suggestion.suggestedAllocation) {
    updates.allocations = suggestion.suggestedAllocation;
  }

  onUpdateGoal(updates);
}

/**
 * Generate acceptance message
 */
export function generateAcceptanceMessage(suggestion: ProactiveSuggestion): string {
  if (suggestion.type === 'contribution-increase' && suggestion.suggestedContribution) {
    return `Got it! I've updated your monthly contribution from $${suggestion.currentContribution} to $${suggestion.suggestedContribution}. This should get us back on track! 🚀`;
  }

  if (suggestion.type === 'allocation-rebalance' && suggestion.suggestedAllocation) {
    return `Perfect! I've rebalanced your portfolio to be more aggressive. Your new allocation should accelerate growth toward your goal. Let's crush it! 💪`;
  }

  if (suggestion.type === 'both') {
    return `Excellent decision! I've increased your monthly to $${suggestion.suggestedContribution} and optimized your allocation. You're set up for success! 🎯`;
  }

  return `Done! Your goal settings have been updated.`;
}
