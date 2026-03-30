/**
 * Chart Utilities - SVG Pie Chart Helper Functions
 * No external charting dependencies - lightweight & customizable
 */

export interface AssetAllocation {
  name: string;
  percentage: number;
  color: string;
}

export interface PieSlice {
  startAngle: number;
  endAngle: number;
  percentage: number;
  name: string;
  color: string;
  x1: number;
  y1: number;
  x2: number;
  y2: number;
  largeArc: boolean;
  cx: number;
  cy: number;
  radius: number;
}

/**
 * Convert percentage to radians
 */
export function percentToRadians(percent: number): number {
  return (percent / 100) * (2 * Math.PI);
}

/**
 * Calculate point on circle given center, radius, and angle
 */
export function getPointOnCircle(
  cx: number,
  cy: number,
  radius: number,
  angle: number
): { x: number; y: number } {
  return {
    x: cx + radius * Math.cos(angle - Math.PI / 2),
    y: cy + radius * Math.sin(angle - Math.PI / 2),
  };
}

/**
 * Generate SVG path for a pie slice
 */
export function generatePiePath(
  cx: number,
  cy: number,
  radius: number,
  startAngle: number,
  endAngle: number
): string {
  const startPoint = getPointOnCircle(cx, cy, radius, startAngle);
  const endPoint = getPointOnCircle(cx, cy, radius, endAngle);
  
  const largeArc = endAngle - startAngle > Math.PI ? 1 : 0;
  
  return [
    `M ${cx} ${cy}`,
    `L ${startPoint.x} ${startPoint.y}`,
    `A ${radius} ${radius} 0 ${largeArc} 1 ${endPoint.x} ${endPoint.y}`,
    `Z`,
  ].join(' ');
}

/**
 * Calculate all pie slices from allocation data
 */
export function calculatePieSlices(
  allocations: AssetAllocation[],
  cx: number,
  cy: number,
  radius: number
): PieSlice[] {
  const slices: PieSlice[] = [];
  let currentAngle = 0;

  for (const allocation of allocations) {
    const sliceAngle = percentToRadians(allocation.percentage);
    const endAngle = currentAngle + sliceAngle;

    const startPoint = getPointOnCircle(cx, cy, radius, currentAngle);
    const endPoint = getPointOnCircle(cx, cy, radius, endAngle);
    const largeArc = sliceAngle > Math.PI;

    slices.push({
      startAngle: currentAngle,
      endAngle: endAngle,
      percentage: allocation.percentage,
      name: allocation.name,
      color: allocation.color,
      x1: startPoint.x,
      y1: startPoint.y,
      x2: endPoint.x,
      y2: endPoint.y,
      largeArc,
      cx,
      cy,
      radius,
    });

    currentAngle = endAngle;
  }

  return slices;
}

/**
 * Calculate label position for pie slice (outside donut for visibility)
 */
export function calculateLabelPosition(
  cx: number,
  cy: number,
  startAngle: number,
  endAngle: number,
  radius: number,
  outerRadius: number
): { x: number; y: number; angle: number } {
  const midAngle = (startAngle + endAngle) / 2;
  const labelRadius = outerRadius + 30;

  return {
    x: cx + labelRadius * Math.cos(midAngle - Math.PI / 2),
    y: cy + labelRadius * Math.sin(midAngle - Math.PI / 2),
    angle: midAngle,
  };
}

/**
 * Parse allocation data from agent message or structured object
 */
export function parseAllocations(data: unknown): AssetAllocation[] {
  // If already in correct format
  if (Array.isArray(data)) {
    return data as AssetAllocation[];
  }

  // If object with allocation property
  if (typeof data === 'object' && data !== null && 'allocations' in data) {
    return (data as { allocations: AssetAllocation[] }).allocations;
  }

  // Fallback: return empty array
  return [];
}

/**
 * Validate allocations sum to 100%
 */
export function validateAllocations(allocations: AssetAllocation[]): boolean {
  const total = allocations.reduce((sum, a) => sum + a.percentage, 0);
  return Math.abs(total - 100) < 0.01; // Allow for floating point errors
}

/**
 * Normalize allocations to exactly 100%
 */
export function normalizeAllocations(
  allocations: AssetAllocation[]
): AssetAllocation[] {
  const total = allocations.reduce((sum, a) => sum + a.percentage, 0);
  if (Math.abs(total - 100) < 0.01) return allocations;

  const scale = 100 / total;
  return allocations.map(a => ({
    ...a,
    percentage: Math.round(a.percentage * scale * 100) / 100,
  }));
}
