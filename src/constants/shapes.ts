// Shape type constants matching Rust backend
export const SHAPE_TYPES = {
  RECTANGLE: 'rectangle',
  CIRCLE: 'circle',
  TRIANGLE: 'triangle',
  TRAPEZOID: 'trapezoid',
  LSHAPE: 'lshape'
} as const;

// Parameter names matching Rust backend
export const PARAM_NAMES = {
  WIDTH: 'width',
  HEIGHT: 'height',
  RADIUS: 'radius',
  BASE: 'base',
  ANGLE: 'angle',
  TOP_WIDTH: 'top_width',
  BOTTOM_WIDTH: 'bottom_width',
  OUTER_WIDTH: 'outer_width',
  OUTER_HEIGHT: 'outer_height',
  INNER_WIDTH: 'inner_width',
  INNER_HEIGHT: 'inner_height'
} as const;

// Rotation options
export const ROTATION_OPTIONS = [0, 90, 180, 270] as const;

// Default parameter values
export const DEFAULT_PARAMETERS = {
  [SHAPE_TYPES.RECTANGLE]: {
    [PARAM_NAMES.WIDTH]: 100,
    [PARAM_NAMES.HEIGHT]: 50
  },
  [SHAPE_TYPES.CIRCLE]: {
    [PARAM_NAMES.RADIUS]: 50
  },
  [SHAPE_TYPES.TRIANGLE]: {
    [PARAM_NAMES.BASE]: 100,
    [PARAM_NAMES.HEIGHT]: 80,
    [PARAM_NAMES.ANGLE]: 60
  },
  [SHAPE_TYPES.TRAPEZOID]: {
    [PARAM_NAMES.TOP_WIDTH]: 60,
    [PARAM_NAMES.BOTTOM_WIDTH]: 100,
    [PARAM_NAMES.HEIGHT]: 50
  },
  [SHAPE_TYPES.LSHAPE]: {
    [PARAM_NAMES.OUTER_WIDTH]: 100,
    [PARAM_NAMES.OUTER_HEIGHT]: 80,
    [PARAM_NAMES.INNER_WIDTH]: 40,
    [PARAM_NAMES.INNER_HEIGHT]: 40
  }
} as const;

// Shape display names
export const SHAPE_DISPLAY_NAMES = {
  [SHAPE_TYPES.RECTANGLE]: 'Rectangle',
  [SHAPE_TYPES.CIRCLE]: 'Circle',
  [SHAPE_TYPES.TRIANGLE]: 'Triangle',
  [SHAPE_TYPES.TRAPEZOID]: 'Trapezoid',
  [SHAPE_TYPES.LSHAPE]: 'L-Shape'
} as const;
