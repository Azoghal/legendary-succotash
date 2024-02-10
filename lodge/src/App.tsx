import SuccotashLogo from './assets/logo.png'
import './App.css'
import Counter from './components/Counter'
import RecipeButton from './components/RecipeButton'
import MuiExample from './components/MuiExample'

function App() {
  return (
    <>
      <div>
        <a href="https://en.wikipedia.org/wiki/Succotash" target="_blank">
          <img src={SuccotashLogo} className="logo" alt="Succotash Logo" />
        </a>
      </div>
      <h1>Legendary. Succotash</h1>
      <Counter val={0}/>
      <RecipeButton/>
      <MuiExample/>
    </>
  )
}

export default App
