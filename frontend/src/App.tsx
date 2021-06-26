import * as React from "react";
import * as api from "./api";
import ShortUrl from "./ShortUrl";
import UrlForm from "./UrlForm";

export default function App() {
  const [isLoading, setIsLoading] = React.useState(false);
  const [url, setUrl] = React.useState("");
  const [shortUrl, setShortUrl] = React.useState("");
  const [error, setError] = React.useState("");

  async function handleSubmit() {
    const trimmedUrl = url.trim();

    if (trimmedUrl) {
      try {
        setError("");
        setIsLoading(true);

        const result = await api.postUrl(trimmedUrl);
        if (result.kind === "right") {
          setShortUrl(
            `http://localhost/api/${encodeURIComponent(result.value)}`
          );
        } else {
          setError(result.value);
        }
      } finally {
        setIsLoading(false);
      }
    }
  }

  function handleReset() {
    setUrl("");
    setShortUrl("");
    setError("");
  }

  return (
    <main>
      <h1>URL Shortener</h1>
      <UrlForm
        url={url}
        onReset={handleReset}
        onSubmit={handleSubmit}
        onUrlChange={setUrl}
      />
      <ShortUrl shortUrl={shortUrl} />
      <p>{isLoading.toString()}</p>
      <p>{error}</p>
    </main>
  );
}
