import "./styles.css";
import { h, render } from "preact";
import { useEffect, useState } from "preact/hooks";
import { AuthClient } from "./auth.ts";

new Worker(globalThis.importMap.resolve("worker")!, {
  type: "module",
});

const authClient = new AuthClient();

function App() {
  const [accountDetails, setAccountDetails] = useState(authClient.getDetails());

  useEffect(() => {
    authClient.onAuthAction(async (status, state) => {
      console.warn(status, state);
      setAccountDetails(authClient.getDetails());
    });
  }, [authClient]);

  function login() {
    authClient.navigateToLogin({ hello: "world" });
  }

  function logout() {
    authClient.navigateToLogout();
  }

  async function refresh() {
    await authClient.refreshAuth();
  }

  async function protectedApi() {
    const resp = await authClient.validate();
    console.log(resp);
  }

  return (
    <div>
      <h1>Site Forge</h1>
      <p id="status">
        Status:{" "}
        <span>
          {accountDetails
            ? `Logged in as ${accountDetails.email}`
            : "Logged out"}{" "}
        </span>
      </p>
      <button onClick={login}>Login</button>
      <button onClick={logout}>Logout</button>
      <button onClick={refresh}>Refresh</button>
      <button onClick={protectedApi}>Protected API</button>
      <code></code>
    </div>
  );
}

render(<App />, document.body);
