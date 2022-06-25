document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('branchSelect').addEventListener('change', (selected) => {
        console.log(selected);
        window.location.href = selected.target.value;
    });
});
