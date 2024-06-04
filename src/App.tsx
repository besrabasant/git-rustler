"use client";
import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

import { Button, Modal } from "flowbite-react";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [openModal, setOpenModal] = useState(false);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container">
        <div className="border-red-500">
          Left Navbar
        </div>
        <div className="border-red-500">
          Main Body Navbar

          <Button onClick={() => setOpenModal(true)}>
            Add Locations
          </Button>
        </div>

        <Modal show={openModal} onClose={() => setOpenModal(false)}>
        <Modal.Header>Terms of Service</Modal.Header>
        <Modal.Body>
          <div className="space-y-6">
            { greetMsg }
          </div>
        </Modal.Body>
        <Modal.Footer>
          <Button onClick={greet}>Greet</Button>
          <Button color="gray" onClick={() => setOpenModal(false)}>
            Cancel
          </Button>
        </Modal.Footer>
      </Modal>
    </div>
  );
}

export default App;
