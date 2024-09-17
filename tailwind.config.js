const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
	content: ["./src/**/*.rs"],
	darkMode: "class", // or 'media' or 'class'
	theme: {
		fontFamily: {
			sans: ["Verdana", ...defaultTheme.fontFamily.sans],
			serif: ["EB Garamond", ...defaultTheme.fontFamily.serif],
			mono: ["Fira Code", ...defaultTheme.fontFamily.mono],
		},
		fontSize: {
			xs: "12px",
			sm: "16px",
			base: "20px",
			lg: "24px",
			xl: "26px",
			"2xl": "30px",
			"3xl": "34px",
			"4xl": "40px",
		},
	},
	plugins: [],
};
