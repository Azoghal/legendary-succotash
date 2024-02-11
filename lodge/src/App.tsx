import SuccotashLogo from './assets/logo.png'
import Card from './components/Card'
import Counter from './components/Counter'
import RecipeButton from './components/RecipeButton'

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
      <div className='columns'>
        <Card title='Tool 1' body='This tool will do a thing'/>
        <Card title='Tool 1' body='This tool will do a thing'/>
        <Card title='Tool 1' body='This tool will do a thing'/>
      </div>
      
    </>
  )
}

export default App
