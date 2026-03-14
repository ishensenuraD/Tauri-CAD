// TypeScript interfaces matching Rust models

export interface Point {
  x: number;
  y: number;
}

export interface Dimension {
  start: Point;
  end: Point;
  offset: number;
  label: string;
}

export interface CanvasData {
  points: Point[];
  dimensions: Dimension[];
}

export interface ShapeConfig {
  shape_type: string;
  parameters: Record<string, number>;
  rotation: 0 | 90 | 180 | 270;
  flip_x: boolean;
  flip_y: boolean;
}

// Shape parameter types
export type ShapeType = 'rectangle' | 'circle' | 'triangle' | 'trapezoid' | 'lshape';

export interface RectangleParameters {
  width: number;
  height: number;
}

export interface CircleParameters {
  radius: number;
}

export interface TriangleParameters {
  base: number;
  height: number;
  angle: number;
}

export interface TrapezoidParameters {
  top_width: number;
  bottom_width: number;
  height: number;
}

export interface LShapeParameters {
  outer_width: number;
  outer_height: number;
  inner_width: number;
  inner_height: number;
}

export type ShapeParameters = 
  | RectangleParameters
  | CircleParameters
  | TriangleParameters
  | TrapezoidParameters
  | LShapeParameters;
