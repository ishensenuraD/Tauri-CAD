import type { RectangleParameters } from '../types';

export interface RectangleShape extends RectangleParameters {
  // Additional rectangle-specific properties can be added here
}

export const defaultRectangle: RectangleParameters = {
  width: 100,
  height: 50
};
