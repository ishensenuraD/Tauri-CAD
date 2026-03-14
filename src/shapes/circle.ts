import type { CircleParameters } from '../types';

export interface CircleShape extends CircleParameters {
  // Additional circle-specific properties can be added here
}

export const defaultCircle: CircleParameters = {
  radius: 50
};
