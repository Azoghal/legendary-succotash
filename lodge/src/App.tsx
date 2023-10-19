import SuccotashLogo from './assets/logo.png'
import './App.css'
import Counter from './components/Counter'

function App() {
  return (
    <>
      <div>
        <a href="https://en.wikipedia.org/wiki/Succotash" target="_blank">
          <img src={SuccotashLogo} className="logo" alt="Succotash Logo" />
        </a>
      </div>
      <h1>Legendary. Succotash</h1>
      <Counter val={55}/>
    </>
  )
}

export default App
