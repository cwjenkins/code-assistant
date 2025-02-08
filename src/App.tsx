import { useState } from 'react';
import { useEffect } from 'react';
import Select from 'react-select'
import Markdown from 'react-markdown'
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { dark } from "react-syntax-highlighter/dist/esm/styles/prism";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [inquiry, setInquiry] = useState("");
  const [modelResponse, setModelResponse] = useState("");
  const [modelOption, setModelOption] = useState("");
  const [modelOptions, setModelOptions] = useState("");

  useEffect(() => {
    getModels();
  }, []);

  async function getModels() {
    let models: string[] = await invoke("list_models");
    console.log(models);

    const createModelOptions = (model: string): { value: string, label: string } => {
      return { value: model, label: model }
    };

    let modelOptions: { value: string, label: string }[] = models.map(createModelOptions);
    console.log(modelOptions);

    setModelOptions(modelOptions);
  }

  async function askModel() {
    console.log("asking a model something");
    console.log(modelOption);
    console.log(inquiry);
    let response = await invoke("ask_model", { model: modelOption, inquiry: inquiry});
    setModelResponse(response);
  }

  const modelOptionChange = (model) => {
    setModelOption(model.value);
  }

  return (
    <main className="container">
      <h1>Welcome to WhiteChat</h1>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
	  askModel();
        }}
      >
        <input
          id="ask-model-input"
          onChange={(e) => setInquiry(e.currentTarget.value)}
          placeholder="Type here to ask the model..."
        />
	<button type="submit">Ask</button>
	<span>Models: </span>
	<Select onChange={modelOptionChange} options={modelOptions} />
      </form>
      <div>Response:
        <Markdown
          components={{
          code({ node, inline, className, children, ...props }) {
            const match = /language-(\w+)/.exec(className || "");
            return !inline && match ? (
               <SyntaxHighlighter
                 style={dark}
                 language={match[1]}
                 PreTag="div"
                  {...props}
                >
               {String(children).replace(/\n$/, "")}
              </SyntaxHighlighter>
            ) : (
            <code className={className} {...props}>
              {children}
            </code>
          );
        },}}
        >{modelResponse}</Markdown>
      </div> 
    </main>
  );
}

export default App;
