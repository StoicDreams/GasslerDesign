/* Display button to navigate to next page */
"use strict"
{
    class Next extends HTMLElement {
        constructor() {
            super();
            let t = this;
            t._insta = document.createElement("webui-flex")
            t._link = document.createElement("webui-flex");
            if (t.parentNode.nodeName === 'P') {
                let p = t.parentNode;
                t.parentNode.parentNode.insertBefore(t, p);
            }
        }
        static get observedAttributes() {
            return ['name', 'href'];
        }
        attributeChangedCallback(property, oldValue, newValue) {
            if (oldValue === newValue) return;
            if (newValue === null || newValue === undefined) {
                delete this[property];
            } else {
                this[property] = newValue;
            }
        }
        connectedCallback() {
            let t = this;
            if (!t.parentNode) { return; }
            t.parentNode.insertBefore(t._insta, t);
            t.parentNode.insertBefore(t._link, t);
            t._insta.innerHTML = webuiApplyAppData(`
<a href="https://www.instagram.com/gasslerdesign" class="f5">

<webui-avatar src="brands instagram"></webui-avatar>

Visit our Instagram

</a>
`);
            t._insta.classList.add('mt-a');
            t._insta.setAttribute('column', true);
            t._insta.setAttribute('align', 'center');
            t._insta.setAttribute('justify', 'center');
            t._link.innerHTML = webuiApplyAppData(`
<a href="${t.href}" class="btn theme-info">

Continue to ${t.name}

<webui-fa icon="right" family="duotone" class="mt-1"></webui-fa>

</a>
`);
            t._link.classList.add('mt-10');
            t._link.setAttribute('column', true);
            t._link.setAttribute('align', 'center');
            t._link.setAttribute('justify', 'center');
        }
        disconnectedCallback() { }
    }
    customElements.define('app-next', Next);
}