import type { ShapeConfig, ShapeType } from '../types';
import { SHAPE_TYPES, PARAM_NAMES, DEFAULT_PARAMETERS } from '../constants/shapes';

// Create a default shape configuration
export function createDefaultShape(shapeType: ShapeType): ShapeConfig {
  return {
    shape_type: shapeType,
    parameters: { ...DEFAULT_PARAMETERS[shapeType] },
    rotation: 0,
    flip_x: false,
    flip_y: false
  };
}

// Validate shape configuration
export function validateShapeConfig(config: ShapeConfig): string[] {
  const errors: string[] = [];

  // Check shape type
  if (!Object.values(SHAPE_TYPES).includes(config.shape_type as ShapeType)) {
    errors.push(`Invalid shape type: ${config.shape_type}`);
  }

  // Check rotation
  if (![0, 90, 180, 270].includes(config.rotation)) {
    errors.push(`Invalid rotation: ${config.rotation}. Must be 0, 90, 180, or 270`);
  }

  // Validate parameters based on shape type
  switch (config.shape_type) {
    case SHAPE_TYPES.RECTANGLE:
      validateRectangleParams(config, errors);
      break;
    case SHAPE_TYPES.CIRCLE:
      validateCircleParams(config, errors);
      break;
    case SHAPE_TYPES.TRIANGLE:
      validateTriangleParams(config, errors);
      break;
    case SHAPE_TYPES.TRAPEZOID:
      validateTrapezoidParams(config, errors);
      break;
    case SHAPE_TYPES.LSHAPE:
      validateLShapeParams(config, errors);
      break;
  }

  return errors;
}

function validateRectangleParams(config: ShapeConfig, errors: string[]) {
  const width = config.parameters[PARAM_NAMES.WIDTH] || 0;
  const height = config.parameters[PARAM_NAMES.HEIGHT] || 0;

  if (width <= 0) {
    errors.push('Rectangle width must be positive');
  }
  if (height <= 0) {
    errors.push('Rectangle height must be positive');
  }
}

function validateCircleParams(config: ShapeConfig, errors: string[]) {
  const radius = config.parameters[PARAM_NAMES.RADIUS] || 0;

  if (radius <= 0) {
    errors.push('Circle radius must be positive');
  }
}

function validateTriangleParams(config: ShapeConfig, errors: string[]) {
  const base = config.parameters[PARAM_NAMES.BASE] || 0;
  const height = config.parameters[PARAM_NAMES.HEIGHT] || 0;
  const angle = config.parameters[PARAM_NAMES.ANGLE] || 0;

  if (base <= 0) {
    errors.push('Triangle base must be positive');
  }
  if (height <= 0) {
    errors.push('Triangle height must be positive');
  }
  if (angle <= 0 || angle >= 180) {
    errors.push('Triangle angle must be between 0 and 180 degrees');
  }
}

function validateTrapezoidParams(config: ShapeConfig, errors: string[]) {
  const topWidth = config.parameters[PARAM_NAMES.TOP_WIDTH] || 0;
  const bottomWidth = config.parameters[PARAM_NAMES.BOTTOM_WIDTH] || 0;
  const height = config.parameters[PARAM_NAMES.HEIGHT] || 0;

  if (topWidth <= 0) {
    errors.push('Trapezoid top width must be positive');
  }
  if (bottomWidth <= 0) {
    errors.push('Trapezoid bottom width must be positive');
  }
  if (height <= 0) {
    errors.push('Trapezoid height must be positive');
  }
}

function validateLShapeParams(config: ShapeConfig, errors: string[]) {
  const outerWidth = config.parameters[PARAM_NAMES.OUTER_WIDTH] || 0;
  const outerHeight = config.parameters[PARAM_NAMES.OUTER_HEIGHT] || 0;
  const innerWidth = config.parameters[PARAM_NAMES.INNER_WIDTH] || 0;
  const innerHeight = config.parameters[PARAM_NAMES.INNER_HEIGHT] || 0;

  if (outerWidth <= 0) {
    errors.push('L-shape outer width must be positive');
  }
  if (outerHeight <= 0) {
    errors.push('L-shape outer height must be positive');
  }
  if (innerWidth <= 0) {
    errors.push('L-shape inner width must be positive');
  }
  if (innerHeight <= 0) {
    errors.push('L-shape inner height must be positive');
  }
  if (innerWidth >= outerWidth) {
    errors.push('L-shape inner width must be less than outer width');
  }
  if (innerHeight >= outerHeight) {
    errors.push('L-shape inner height must be less than outer height');
  }
}

// Clone shape configuration
export function cloneShapeConfig(config: ShapeConfig): ShapeConfig {
  return {
    ...config,
    parameters: { ...config.parameters }
  };
}

// Get required parameters for a shape type
export function getRequiredParameters(shapeType: ShapeType): string[] {
  switch (shapeType) {
    case SHAPE_TYPES.RECTANGLE:
      return [PARAM_NAMES.WIDTH, PARAM_NAMES.HEIGHT];
    case SHAPE_TYPES.CIRCLE:
      return [PARAM_NAMES.RADIUS];
    case SHAPE_TYPES.TRIANGLE:
      return [PARAM_NAMES.BASE, PARAM_NAMES.HEIGHT, PARAM_NAMES.ANGLE];
    case SHAPE_TYPES.TRAPEZOID:
      return [PARAM_NAMES.TOP_WIDTH, PARAM_NAMES.BOTTOM_WIDTH, PARAM_NAMES.HEIGHT];
    case SHAPE_TYPES.LSHAPE:
      return [
        PARAM_NAMES.OUTER_WIDTH,
        PARAM_NAMES.OUTER_HEIGHT,
        PARAM_NAMES.INNER_WIDTH,
        PARAM_NAMES.INNER_HEIGHT
      ];
    default:
      return [];
  }
}
