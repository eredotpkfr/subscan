// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="index.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="user-guide/index.html"><strong aria-hidden="true">2.</strong> User Guide</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="user-guide/quickstart/index.html"><strong aria-hidden="true">2.1.</strong> Quickstart</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="user-guide/quickstart/install.html"><strong aria-hidden="true">2.1.1.</strong> Install</a></li><li class="chapter-item expanded "><a href="user-guide/quickstart/usage/index.html"><strong aria-hidden="true">2.1.2.</strong> Usage</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="user-guide/quickstart/usage/cli.html"><strong aria-hidden="true">2.1.2.1.</strong> CLI</a></li><li class="chapter-item expanded "><a href="user-guide/quickstart/usage/docker.html"><strong aria-hidden="true">2.1.2.2.</strong> Docker</a></li><li class="chapter-item expanded "><a href="user-guide/quickstart/usage/crate.html"><strong aria-hidden="true">2.1.2.3.</strong> Crate</a></li></ol></li><li class="chapter-item expanded "><a href="user-guide/commands/index.html"><strong aria-hidden="true">2.1.3.</strong> Commands</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="user-guide/commands/scan.html"><strong aria-hidden="true">2.1.3.1.</strong> scan</a></li><li class="chapter-item expanded "><a href="user-guide/commands/brute.html"><strong aria-hidden="true">2.1.3.2.</strong> brute</a></li><li class="chapter-item expanded "><a href="user-guide/commands/module.html"><strong aria-hidden="true">2.1.3.3.</strong> module</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="user-guide/environments.html"><strong aria-hidden="true">2.2.</strong> Environments</a></li></ol></li><li class="chapter-item expanded "><a href="development/index.html"><strong aria-hidden="true">3.</strong> Development</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="development/environment.html"><strong aria-hidden="true">3.1.</strong> Setup Environment</a></li><li class="chapter-item expanded "><a href="development/components/index.html"><strong aria-hidden="true">3.2.</strong> Components</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="development/components/requesters.html"><strong aria-hidden="true">3.2.1.</strong> Requesters</a></li><li class="chapter-item expanded "><a href="development/components/extractors.html"><strong aria-hidden="true">3.2.2.</strong> Extractors</a></li><li class="chapter-item expanded "><a href="development/components/module.html"><strong aria-hidden="true">3.2.3.</strong> Subscan Module</a></li></ol></li><li class="chapter-item expanded "><a href="development/generics/index.html"><strong aria-hidden="true">3.3.</strong> Generic Modules</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="development/generics/integration.html"><strong aria-hidden="true">3.3.1.</strong> Integration</a></li><li class="chapter-item expanded "><a href="development/generics/engine.html"><strong aria-hidden="true">3.3.2.</strong> Search Engine</a></li></ol></li><li class="chapter-item expanded "><a href="development/integration.html"><strong aria-hidden="true">3.4.</strong> Integrate Your Module Step by Step</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
