export function getAuthorFilter() {
    return document.getElementById('authorFilter').value.toLowerCase() ?? "";
}

export function getTitleFilter() {
    return document.getElementById('titleFilter').value.toLowerCase() ?? "";
}
export function filterHighlights(highlights) {
    const bookFilter = getTitleFilter()
    const authorFilter = getAuthorFilter()

    return highlights.filter(highlight =>
        (highlight.title.toLowerCase().includes(bookFilter)) &&
        (highlight.author != null && highlight.author.toLowerCase().includes(authorFilter))
    );
}

export function displayResults(results) {
    const resultsDiv = document.getElementById('results');
    resultsDiv.innerHTML = results
        .map((h, index) => {
            let color = (index % 2) === 0 ? 'AliceBlue' : 'LightGray'
            return `
            <div class="highlight" style="padding: 2ch; background-color: ${color};">
                <p><strong>Title:</strong> ${h.title}</p>
                <p><strong>Author:</strong> ${h.author}</p>
                <p>${h.content}</p>
            </div>
        `
        })
        .join('');
}
