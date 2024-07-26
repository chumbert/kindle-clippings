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


    export function exportResults() {
    // Exporting logic for the results
    // This is a placeholder function, adjust according to your needs
    const bookFilter = document.getElementById('bookFilter').value;
    const authorFilter = document.getElementById('authorFilter').value;
    
    const exportContent = highlights.map(
        h => `${h.book}|${h.author}|${h.text}`).join('\n');
    const blob = new Blob([exportContent], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    
    const a = document.createElement('a');
    a.href = url;
    a.download = `highlights-${bookFilter || 'all'}-${authorFilter || 'all'}.txt`;
    a.click();
    URL.revokeObjectURL(url);
}

