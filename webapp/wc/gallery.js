/* Display gallery images */
"use strict"
{
    class Gallery extends HTMLElement {
        constructor() {
            super();
            const t = this;
            if (t.parentNode.nodeName === 'P') {
                let p = t.parentNode;
                t.parentNode.parentNode.insertBefore(t, p);
                p.remove();
            }
            t.flexImage = document.createElement('webui-flex');
            t.flexName = document.createElement('webui-flex');
            t.cards = document.createElement('webui-cards');
        }
        static get observedAttributes() {
            return ['src'];
        }
        attributeChangedCallback(property, oldValue, newValue) {
            if (oldValue === newValue) return;
            if (newValue === null || newValue === undefined) {
                delete this[property];
            } else {
                this[property] = newValue;
            }
            switch (property) {
                case 'src':
                    break;
            }
        }
        async loadGallery() {
            let result = await fetch('/gallery.json');
            if (!result.ok) { return; }
            let images = await result.json();
            if (!images.length) { return; }
            let t = this;
            t.parentNode.insertBefore(t.flexImage, t);
            t.parentNode.insertBefore(t.flexName, t);
            t.parentNode.insertBefore(t.cards, t);

            t.currentImage = images[0];
            t.flexImage.setAttribute('justify', 'center');
            t.flexImage.setAttribute('align', 'center');
            t.flexImage.setAttribute('column', 'true');
            t.flexImage.classList.add('pa-2');
            t.flexImage.style.height = 'calc(0.8 * var(--main-height))';

            let img = document.createElement('img');
            img.setAttribute('data-subscribe', 'page-image');
            img.setAttribute('data-set', 'src');
            t.flexImage.appendChild(img);

            t.flexName.setAttribute('justify', 'center');
            t.flexName.classList.add('pa-1', 'ma-1');
            let nm = document.createElement('p');
            t.flexName.appendChild(nm);
            nm.setAttribute('data-subscribe', 'page-image-name');
            nm.setAttribute('data-set', 'innerHTML');

            t.cards.classList.add('mb-5');

            let cardTemplate = document.createElement('webui-card');
            images.forEach(image => {
                let card = cardTemplate.cloneNode(false);
                t.cards.appendChild(card);
                card.setAttribute('title', image.name);
                let ca = document.createElement('webui-avatar');
                card.appendChild(ca);
                card.style.cursor = 'pointer';
                ca.setAttribute('src', image.src);
                ca.style.fontSize = '4em';
                card.addEventListener('click', _ev => {
                    webuiSetData('page-image', image.src);
                    webuiSetData('page-image-name', image.name);
                });
            });

            webuiSetData('page-image', t.currentImage.src);
            webuiSetData('page-image-name', t.currentImage.name);
        }
        connectedCallback() {
            let t = this;
            this.loadGallery();
        }
        disconnectedCallback() { }
    }
    customElements.define('app-gallery', Gallery);
}