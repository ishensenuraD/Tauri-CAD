import type { LShapeParameters } from '../types';

export interface LShapeShape extends LShapeParameters {
  // Additional L-shape-specific properties can be added here
}

export const defaultLShape: LShapeParameters = {
  outer_width: 100,
  outer_height: 80,
  inner_width: 40,
  inner_height: 40
};
