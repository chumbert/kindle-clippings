import { parse_clippings } from './pkg/kindle_clippings.js';

document.getElementById('filterButton').addEventListener('click', filterHighlights);
document.getElementById('exportButton').addEventListener('click', exportResults);


// document.getElementById('fileInput').addEventListener('change', function(event) {
//     const file = event.target.files[0];
//     if (file) {
//         const reader = new FileReader();
//         reader.onload = function(e) {
//             highlights = parse_clippings(e.target.result);
//             displayResults(highlights);
//             console.log(highlights);
//         };
//         reader.readAsText(file);
//     }
// });

function filterHighlights() {
    const bookFilter = document.getElementById('bookFilter').value.toLowerCase();
    const authorFilter = document.getElementById('authorFilter').value.toLowerCase();

    const filteredHighlights = highlights.filter(highlight => 
        (!bookFilter || highlight.book.toLowerCase().includes(bookFilter)) &&
        (!authorFilter || highlight.author.toLowerCase().includes(authorFilter))
    );

    displayResults(filteredHighlights);
}

function displayResults(highlights) {
    const resultsDiv = document.getElementById('results');
    resultsDiv.innerHTML = highlights.map(h => `
        <div class="highlight">
            <p><strong>Book:</strong> ${h.book}</p>
            <p><strong>Author:</strong> ${h.author}</p>
            <p>${h.text}</p>
        </div>
    `).join('');
}

function exportResults() {
    // Exporting logic for the results
    // This is a placeholder function, adjust according to your needs
    const bookFilter = document.getElementById('bookFilter').value;
    const authorFilter = document.getElementById('authorFilter').value;
    
    const exportContent = highlights.map(h => `${h.book}|${h.author}|${h.text}`).join('\n');
    const blob = new Blob([exportContent], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    
    const a = document.createElement('a');
    a.href = url;
    a.download = `highlights-${bookFilter || 'all'}-${authorFilter || 'all'}.txt`;
    a.click();
    URL.revokeObjectURL(url);
}

