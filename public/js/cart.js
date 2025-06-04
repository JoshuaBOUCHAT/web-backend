document.addEventListener('DOMContentLoaded', function () {
    // Attach event listeners to all quantity input fields
    const overlay = document.getElementById('modal-overlay');
    const content = document.getElementById('modal-content');
    function openModal(html) {
        content.innerHTML = html;
        overlay.style.display = 'flex';
    }

    function closeModal() {
        overlay.style.display = 'none';
        window.location.reload();
    }

    overlay.addEventListener('click', e => {
        if (e.target === overlay || e.target === closeBtn) {
            closeModal();
        }
    });
    document.getElementById('cart-order').addEventListener('click', e => {
        console.log("test");
        e.preventDefault();
        fetch("/cart/order")
            .then(r => r.text())
            .then(html => { openModal(html); console.log(html) }).catch(err => console.error(err));
    });



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
                    window.location.reload();
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
    document.querySelectorAll('.trash-img-btn').forEach(img => {
        img.addEventListener('click', function () {
            // Trouver le conteneur du produit
            const container = this.closest('.product-cart');
            if (!container) return;

            // Trouver le form spécifique à ce produit
            const form = container.querySelector('form');
            if (!form) return;

            // Confirmation et suppression
            if (confirm("Voulez-vous supprimer cet article de votre panier ?")) {
                fetch(form.action, { method: 'DELETE' })
                    .then(r => r.text())
                    .then(html => {
                        alert(html);
                        // Option 1 : recharger la page
                        window.location.reload();

                        // Option 2 (facultatif) : retirer l'élément du DOM sans recharger
                        // container.remove();
                    })
                    .catch(err => alert("Une erreur est survenue : " + err));
            }
        });
    });
    document.addEventListener('submit', (e) => {
        if (e.target && e.target.matches('form[action="/cart/order"]')) {
            e.preventDefault();
            const form = e.target;
            const url = form.action;
            const data = form_to_url_encoded(form);
            
            fetch(url, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded',
                },
                body: data
            })
            .then(response => {
                if (response.ok) {
                    window.location.href = '/dashboard';
                } else {
                    return response.text().then(text => {
                        const content = document.getElementById('modal-content');
                        if (content) content.innerHTML = text || 'Erreur lors de la commande';
                    });
                }
            })
            .catch(error => {
                const content = document.getElementById('modal-content');
                if (content) content.innerHTML = 'Erreur de connexion au serveur';
                console.error('Erreur:', error);
            });
        }
    });


    /*document.getElementById("#order-confirm").addEventListener('click', e => {
        e.preventDefault();
    });*/
});
function form_to_url_encoded(form) {
    const data = new FormData(form);
    const urlEncodedData = new URLSearchParams();

    for (const [key, value] of data.entries()) {
        urlEncodedData.append(key, value);
    }
    return urlEncodedData;
}