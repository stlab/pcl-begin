document.addEventListener('DOMContentLoaded', function() {
    console.log('D3 Graph script loaded');
    
    // Check if D3.js is available
    if (typeof d3 === 'undefined') {
        console.error('D3.js is not loaded!');
        return;
    }
    
    console.log('D3.js is available, version:', d3.version);
    
    // Wait a bit to ensure the container is rendered
    setTimeout(function() {
        console.log('Looking for d3-graph-container...');
        const container = document.getElementById('d3-graph-container');
        if (!container) {
            console.error('Container d3-graph-container not found!');
            return;
        }
        console.log('Container found:', container);
        
        // Clear any existing content
        container.innerHTML = '';
        
        // Set up dimensions
        const width = container.clientWidth;
        const height = container.clientHeight;
        
        // Create SVG
        console.log('Creating SVG with dimensions:', width, 'x', height);
        const svg = d3.select('#d3-graph-container')
            .append('svg')
            .attr('width', width)
            .attr('height', height);
        
        console.log('SVG created:', svg.node());
        
        // Sample data - a few nodes and links
        const nodes = [
            { id: 'Node 1', group: 1 },
            { id: 'Node 2', group: 1 },
            { id: 'Node 3', group: 2 },
            { id: 'Node 4', group: 2 },
            { id: 'Node 5', group: 3 }
        ];
        
        const links = [
            { source: 'Node 1', target: 'Node 2' },
            { source: 'Node 2', target: 'Node 3' },
            { source: 'Node 3', target: 'Node 4' },
            { source: 'Node 4', target: 'Node 5' },
            { source: 'Node 1', target: 'Node 4' }
        ];
        
        // Color scale
        const color = d3.scaleOrdinal(d3.schemeCategory10);
        
        // Create force simulation
        console.log('Creating force simulation with', nodes.length, 'nodes and', links.length, 'links');
        const simulation = d3.forceSimulation(nodes)
            .force('link', d3.forceLink(links).id(d => d.id).distance(100))
            .force('charge', d3.forceManyBody().strength(-300))
            .force('center', d3.forceCenter(width / 2, height / 2));
        
        // Add links
        const link = svg.append('g')
            .attr('stroke', '#999')
            .attr('stroke-opacity', 0.6)
            .selectAll('line')
            .data(links)
            .join('line')
            .attr('stroke-width', 2);
        
        // Add nodes
        const node = svg.append('g')
            .attr('stroke', '#fff')
            .attr('stroke-width', 1.5)
            .selectAll('circle')
            .data(nodes)
            .join('circle')
            .attr('r', 8)
            .attr('fill', d => color(d.group))
            .call(d3.drag()
                .on('start', dragstarted)
                .on('drag', dragged)
                .on('end', dragended));
        
        // Add labels
        const labels = svg.append('g')
            .selectAll('text')
            .data(nodes)
            .join('text')
            .text(d => d.id)
            .attr('font-size', 12)
            .attr('font-family', 'Arial, sans-serif')
            .attr('text-anchor', 'middle')
            .attr('dy', '.35em')
            .attr('fill', '#333');
        
        // Update positions on simulation tick
        simulation.on('tick', () => {
            link
                .attr('x1', d => d.source.x)
                .attr('y1', d => d.source.y)
                .attr('x2', d => d.target.x)
                .attr('y2', d => d.target.y);
            
            node
                .attr('cx', d => d.x)
                .attr('cy', d => d.y);
            
            labels
                .attr('x', d => d.x)
                .attr('y', d => d.y);
        });
        
        // Drag functions
        function dragstarted(event, d) {
            if (!event.active) simulation.alphaTarget(0.3).restart();
            d.fx = d.x;
            d.fy = d.y;
        }
        
        function dragged(event, d) {
            d.fx = event.x;
            d.fy = event.y;
        }
        
        function dragended(event, d) {
            if (!event.active) simulation.alphaTarget(0);
            d.fx = null;
            d.fy = null;
        }
        
        console.log('D3.js force graph setup complete!');
    }, 100);
}); 
