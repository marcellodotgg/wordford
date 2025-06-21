document.addEventListener("keydown", function (e) {
  if (e.ctrlKey && e.key.toLowerCase() === "k") {
    const searchBar = document.getElementById("search_bar");
    if (searchBar && document.activeElement !== searchBar) {
      e.preventDefault();
      searchBar.focus();
      searchBar.select();
    }
  }
});
