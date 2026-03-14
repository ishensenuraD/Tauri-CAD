export interface TrapezoidShape {
  topWidth: number;
  bottomWidth: number;
  height: number;
}

export const defaultTrapezoid: TrapezoidShape = {
  topWidth: 60,
  bottomWidth: 100,
  height: 50
};
