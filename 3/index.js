let isLight = true;

function themeSwitch() {
	isLight = !isLight;
	toggle.innerText = isLight ? "🌞" : "🌚";
	let rootElement = document.body;
	rootElement.classList.toggle("lightMode");
}

const toggle = document.querySelector("#theme_toggle");
toggle.addEventListener("click", themeSwitch);