import type { TrapezoidParameters } from '../types';

export interface TrapezoidShape extends TrapezoidParameters {
  // Additional trapezoid-specific properties can be added here
}

export const defaultTrapezoid: TrapezoidParameters = {
  top_width: 60,
  bottom_width: 100,
  height: 50
};
