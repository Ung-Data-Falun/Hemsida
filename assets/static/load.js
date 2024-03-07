let replace_elements = document.getElementsByClassName("replace");

for (let i = 0; i < replace_elements.length; i++) {
	let current_element = replace_elements[i];
	let url = current_element.attributes.href.value;
	fetch(url).then((value) => value.text()).then((value) => {
		current_element.innerHTML = value;
	});
}


