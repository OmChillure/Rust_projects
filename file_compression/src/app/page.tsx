"use client"
import { useState } from 'react';

export default function FileUploadPage() {
  const [file, setFile] = useState<File | null>(null);
  const [message, setMessage] = useState('');

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      setFile(e.target.files[0]);
    }
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!file) {
      setMessage('Please select a file first.');
      return;
    }

    const formData = new FormData();
    formData.append('file', file);

    const response = await fetch('http://localhost:8080/upload', {
      method: 'POST',
      body: formData,
    });

    if (response.ok) {
      const result = await response.text();
      setMessage(result);
    } else {
      setMessage('File upload failed.');
    }
  };

  return (
    <div style={{ padding: '20px', maxWidth: '600px', margin: '0 auto' }}>
      <h1>File Upload and Compression</h1>
      <form onSubmit={handleSubmit}>
        <input type="file" onChange={handleFileChange} />
        <button type="submit" style={{ marginTop: '10px' }}>
          Upload
        </button>
      </form>
      {message && <p>{message}</p>}
    </div>
  );
}
