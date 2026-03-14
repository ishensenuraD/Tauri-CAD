import CanvasPreview from './components/CanvasPreview/CanvasPreview'
import ShapeEditor from './components/ShapeEditor/ShapeEditor'
import ShapeLibrary from './components/ShapeLibrary/ShapeLibrary'
import ExportPanel from './components/ExportPanel/ExportPanel'
import './App.css'

function App() {
  return (
    <div className="app">
      <header className="app-header">
        <h1>Tauri CAD Shape Designer</h1>
      </header>
      
      <main className="app-main">
        <div className="sidebar">
          <ShapeLibrary />
          <ShapeEditor />
          <ExportPanel />
        </div>
        
        <div className="canvas-container">
          <CanvasPreview />
        </div>
      </main>
    </div>
  )
}

export default App
