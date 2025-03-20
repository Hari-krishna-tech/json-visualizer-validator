import { useState, useEffect } from "react";
import "./App.css";
import { Code2, RefreshCw, FileCode, FileJson } from "lucide-react";
import Navbar from "./components/Navbar";
import Editor from "./components/Editor";
import Converter from "./components/Converter";
import TypeGenerator from "./components/TypeGenerator";
import SchemaTools from "./components/SchemaTools";

type Tab = "editor" | "converter" | "typeGenerator" | "schema";

function App() {
  const [isDarkMode, setIsDarkMode] = useState(false);
  const [activeTab, setActiveTab] = useState<Tab>("editor");

  const toggleTheme = () => {
    setIsDarkMode(!isDarkMode);
    document.documentElement.classList.toggle("dark");
  };

  const tabs = [
    { id: "editor", label: "Editor", icon: Code2 },
    { id: "converter", label: "Converter", icon: RefreshCw },
    { id: "typeGenerator", label: "Type Generator", icon: FileCode },
    { id: "schema", label: "JSON Schema", icon: FileJson },
  ] as const;

  useEffect(() => {
    const handleKeyDown = (event: any) => {
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "s") {
        event.preventDefault(); // Prevent browser save action
        console.log("Ctrl + S detected, but prevented the default behavior.");
        // Call your custom save function here
      }
    };

    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, []);

  const renderContent = () => {
    switch (activeTab) {
      case "editor":
        return <Editor isDarkMode={isDarkMode} />;
      case "converter":
        return <Converter isDarkMode={isDarkMode} />;
      case "typeGenerator":
        return <TypeGenerator isDarkMode={isDarkMode} />;
      case "schema":
        return <SchemaTools isDarkMode={isDarkMode} />;
      default:
        return <Editor isDarkMode={isDarkMode} />;
    }
  };

  return (
    <div className={`min-h-screen ${isDarkMode ? "dark" : ""}`}>
      <div className="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors duration-200">
        <Navbar isDarkMode={isDarkMode} onThemeToggle={toggleTheme} />

        <main className="container mx-auto px-4 py-6">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg overflow-hidden">
            <div className="border-b border-gray-200 dark:border-gray-700">
              <nav className="flex space-x-2 px-4" aria-label="Tabs">
                {tabs.map(({ id, label, icon: Icon }) => (
                  <button
                    key={id}
                    onClick={() => setActiveTab(id)}
                    className={`
                      py-4 px-6 inline-flex items-center gap-2 border-b-2 font-medium text-sm
                      ${
                        activeTab === id
                          ? "border-blue-500 text-blue-600 dark:text-blue-400 dark:border-blue-400"
                          : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-300"
                      }
                    `}
                  >
                    <Icon className="w-4 h-4" />
                    {label}
                  </button>
                ))}
              </nav>
            </div>

            <div className="p-6">{renderContent()}</div>
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;
