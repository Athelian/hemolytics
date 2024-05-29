import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  let xmlDoc
  async function greet() {
    const a: string = await invoke("get_xml");
    console.log(a)    
  }

  return (
    <div className="container">
      <h1
        onClick={() => {
          greet();
        }}
      >
        Welcome to Tauri!
      </h1>
    </div>
  );
}

export default App;
