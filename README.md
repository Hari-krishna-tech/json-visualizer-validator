

# JSON Visualizer and Validator

A powerful web-based tool for visualizing, validating, and formatting JSON data with tree and graph visualization capabilities.

## Features

- **JSON Editor**
  - Real-time JSON validation
  - Syntax highlighting
  - Auto-formatting on entry
  - Error detection and highlighting

- **Multiple Input Methods**
  - Direct text input
  - File upload support
  - URL import functionality

- **Visualization Options**
  - Interactive tree view
  - Dynamic graph visualization
  - Collapsible nodes
  - Zoom and pan controls

- **Export Capabilities**
  - Export visualizations as PNG
  - Save as JPEG format
  - Download as SVG
  - Copy formatted JSON to clipboard

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/json-visualizer.git
cd json-visualizer
```

2. Install dependencies:
```bash
npm install
```

3. Start the development server:
```bash
npm run dev
```

## Usage

1. **Input JSON Data**
   - Type or paste JSON directly into the editor
   - Upload a JSON file using the file picker
   - Enter a URL containing JSON data

2. **Validate and Format**
   - JSON is automatically validated as you type
   - Press Enter or click the Format button to beautify the JSON
   - Error messages appear below the editor if JSON is invalid

3. **Visualize**
   - Switch between Tree and Graph views using the toggle buttons
   - Click nodes to expand/collapse sections
   - Use mouse wheel to zoom in/out
   - Drag to pan around the visualization

4. **Export**
   - Click the Export button to open export options
   - Select desired format (PNG/JPEG/SVG)
   - Choose export quality and dimensions
   - Save to your local machine

## Tech Stack

- Frontend Framework: React/Next.js
- JSON Parser: [json-parser-library]
- Visualization: D3.js
- Styling: Tailwind CSS
- Code Editor: Monaco Editor

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Development

### Project Structure
```
json-visualizer/
├── src/
│   ├── components/
│   │   ├── Editor/
│   │   ├── Visualizer/
│   │   └── ExportTools/
│   ├── utils/
│   └── pages/
├── public/
└── tests/
```

### Running Tests
```bash
npm run test
```

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Contact

Your Name - [@yourusername](https://twitter.com/yourusername)
Project Link: [https://github.com/yourusername/json-visualizer](https://github.com/yourusername/json-visualizer)
