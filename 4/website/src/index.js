// preference
const theme = localStorage.getItem('theme')

if (theme === null && matchMedia?.('(prefers-color-scheme: dark)').matches) {
	// initial use
	localStorage.setItem('theme', 'dark')
}

const toggle = document.querySelector('#theme_toggle')
toggle.addEventListener('click', themeSwitch)

updateTheme()

function themeSwitch() {
	const theme = localStorage.getItem('theme')
	localStorage.setItem('theme', theme === 'dark' ? 'light' : 'dark')
	window.dispatchEvent(new Event('storage'))
}

function updateTheme() {
	const theme = localStorage.getItem('theme')
	toggle.innerText = theme === 'light' ? '🌚' : '🌞'
	if (theme === 'light') {
		document.body.classList.add('lightMode')
	} else {
		document.body.classList.remove('lightMode')
	}
}

addEventListener('storage', updateTheme)

const url = 'http://localhost:8080/'

fetch(url, {credentials: 'include'})
	.then(console.log)
