import React, { useRef, useEffect } from 'react';

const CanvasPreview: React.FC = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    // Set up canvas styling
    ctx.strokeStyle = '#000000';
    ctx.lineWidth = 1;
    
    // TODO: Implement shape rendering
    // This will be implemented in Phase 8
  }, []);

  return (
    <div className="canvas-preview">
      <h3>Canvas Preview</h3>
      <canvas
        ref={canvasRef}
        width={600}
        height={400}
        style={{ border: '1px solid #ccc' }}
      />
    </div>
  );
};

export default CanvasPreview;
