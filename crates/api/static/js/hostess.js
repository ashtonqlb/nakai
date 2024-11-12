// let uploadForm = new Dropzone("div.upload-form", {
//   chunking: true,
//   parallelChunkUploads: true,
//   retryChunks: true,
//   maxFilesize: 64000000,
// });

// Function to apply the correct theme
function applyTheme(isDarkMode) {
  document.documentElement.setAttribute("data-theme", isDarkMode ? "dark" : "light");
}

// Check the user's current theme preference
const prefersDarkScheme = window.matchMedia("(prefers-color-scheme: dark)");

// Apply the theme on initial load
applyTheme(prefersDarkScheme.matches);

// Listen for changes in the theme preference
prefersDarkScheme.addEventListener("change", (event) => {
  applyTheme(event.matches);
});

// Optionally, you can also handle the case when the page is loaded and the user has set a theme manually.
document.addEventListener("DOMContentLoaded", () => {
  applyTheme(prefersDarkScheme.matches);
});
