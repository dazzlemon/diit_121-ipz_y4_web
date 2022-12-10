// preference
const theme = localStorage.getItem('theme')

if (theme === null && matchMedia?.('(prefers-color-scheme: dark)').matches) {
	// initial use
	localStorage.setItem('theme', 'dark')
}

const toggle = document.querySelector('#theme_toggle')
toggle.addEventListener('click', themeSwitch)

const resetCounterButton = document.querySelector('#reset_counter')
resetCounterButton.addEventListener('click', resetCounter)

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

function resetCounter() {
	fetch('http://localhost:8080/reset', {credentials: 'include'})
		.then(response => response.text())
		.then(count => document.querySelector('#visit_counter').innerText = count)
}

addEventListener('storage', updateTheme)

const url = 'http://localhost:8080/'

fetch(url, {credentials: 'include'})
	.then(response => response.text())
	.then(count => document.querySelector('#visit_counter').innerText = count)
