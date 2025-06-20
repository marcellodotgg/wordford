document.addEventListener("keydown", function (e) {
  if (e.key === "/") {
    const searchBar = document.getElementById("search_bar");
    if (searchBar && document.activeElement !== searchBar) {
      e.preventDefault();
      searchBar.focus();
      searchBar.select();
    }
  }
});
