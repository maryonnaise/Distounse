import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./custom.css";

function App() {
  const [km, setKm] = useState(0);
  const [showModal, setShowModal] = useState(false);

  useEffect(() => {
    const interval = setInterval(() => {
      invoke("get_mouse_distance").then(setKm).catch(console.error);
      //prettier-ignore
      invoke("get_mouse_distance").then(setKm).catch(console.error);
    }, 1000); // aggiorna ogni 1 secondo

    return () => clearInterval(interval);
  }, [km]);

  const handleResetConfirm = () => {
    invoke("reset_mouse_distance")
      .then(() => {
        setKm(0);
        setShowModal(false);
      })
      .catch(console.error);
  };

  return (
    <div className="container">
      <div className="card">
        <h1>Mouse Distance Tracker</h1>
        <p>
          Hai percorso: <span className="distance">{km.toFixed(3)} km</span>
        </p>
        <button onClick={() => setShowModal(true)}>Azzera contatore</button>
      </div>

      {showModal && (
        <div className="modal-overlay">
          <div className="modal">
            <h2>Sei sicuro?</h2>
            <p>Vuoi davvero azzerare il contatore?</p>
            <div className="modal-buttons">
              <button className="cancel" onClick={() => setShowModal(false)}>
                Annulla
              </button>
              <button className="confirm" onClick={handleResetConfirm}>
                Sì, azzera
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
