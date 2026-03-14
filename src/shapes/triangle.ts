export interface TriangleShape {
  base: number;
  height: number;
  angle: number; // apex angle in degrees
}

export const defaultTriangle: TriangleShape = {
  base: 100,
  height: 80,
  angle: 60
};
