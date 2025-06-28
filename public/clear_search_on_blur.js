document.addEventListener("DOMContentLoaded", function () {
  const wrapper = document.getElementById("search_wrapper");
  const input = document.getElementById("search_bar");
  const results = document.getElementById("search_results");

  document.addEventListener("click", function (event) {
    if (!wrapper.contains(event.target)) {
      clearSearch();
    }
  });

  if (wrapper) {
    wrapper.addEventListener("focusout", function () {
      setTimeout(() => {
        if (!wrapper.contains(document.activeElement)) {
          clearSearch();
        }
      }, 0);
    });
  }

  if (input) {
    input.addEventListener("keydown", function (event) {
      if (event.key === "Escape") {
        clearSearch();
        input.blur();
      }
    });
  }

  function clearSearch() {
    input.value = "";
    results.innerHTML = "";
  }
});
