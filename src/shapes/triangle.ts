import type { TriangleParameters } from '../types';

export interface TriangleShape extends TriangleParameters {
  // Additional triangle-specific properties can be added here
}

export const defaultTriangle: TriangleParameters = {
  base: 100,
  height: 80,
  angle: 60
};
