import SuccotashLogo from './assets/logo.png'
import './App.css'

function App() {
  return (
    <>
      <div>
        <a href="https://en.wikipedia.org/wiki/Succotash" target="_blank">
          <img src={SuccotashLogo} className="logo" alt="Succotash Logo" />
        </a>
      </div>
      <h1>Legendary. Succotash</h1>
    </>
  )
}

export default App
