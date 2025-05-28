document.addEventListener('DOMContentLoaded', function () {
    // Attach event listeners to all quantity input fields



    document.querySelectorAll('.quantity-input').forEach(input => {
        input.addEventListener('change', function () {
            const form = this.closest('form');
            const qty = parseFloat(this.value); // ← bien parser
            const productCart = form.closest('.product-cart');
            const price = parseFloat(productCart.querySelector('.price').textContent.replace('€', '').trim());

            if (qty === 0) {
                if (confirm("Vous avez sélectionné la quantité 0. Voulez-vous supprimer cet article ?")) {
                    fetch(form.action, { method: 'DELETE' })
                        .then(r => r.text())
                        .then(html => alert(html))
                        .catch(err => alert("Une erreur est survenue : " + err));
                } else {
                    this.value = 1;
                }
                return;
            }

            // ✅ MAJ du total
            const totalElem = productCart.querySelector('.products-price');
            if (totalElem) {
                const total = (qty * price).toFixed(2);
                totalElem.textContent = `Total: ${total}€`;
            }

            // PUT request
            fetch(form.action + '/' + qty, {
                method: 'PUT',
            })
                .then(response => response.text())
                .then(data => {
                    // facultatif : traitement post-MAJ
                })
                .catch(error => {
                    alert('Erreur : ' + error);
                });
        });
    });
});