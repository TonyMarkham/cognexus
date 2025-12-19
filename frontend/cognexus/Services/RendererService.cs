using Microsoft.JSInterop;

namespace CognexusBlazor.Services;

public class RendererService
{
    private readonly IJSRuntime _jsRuntime;
    private IJSObjectReference? _renderer;
    private bool _isInitialized;
    
    public RendererService(IJSRuntime jsRuntime)
    {
        _jsRuntime = jsRuntime;
    }

    public async Task InitializeAsync(string canvasId, int width, int height)
    {
        if (_isInitialized)
            return;

        var helper = await _jsRuntime.InvokeAsync<IJSObjectReference>(
            "import",
            "./js/renderer-helper.js"
        );

        var canvas = await _jsRuntime.InvokeAsync<IJSObjectReference>(
            "eval",
            $"document.getElementById('{canvasId}')"
        );

        _renderer = await helper.InvokeAsync<IJSObjectReference>(
            "createRenderer",
            canvas,
            width,
            height
        );
    
        _isInitialized = true;
    }

    public async Task HandleDrawQuadCommandAsync(byte[] bytes)
    {
        if(_renderer == null)
            throw new InvalidOperationException("Renderer is not initialized");
        
        await _renderer.InvokeVoidAsync("handle_draw_quad_command", bytes);
    }
    
    public async Task HandlePanCameraCommandAsync(byte[] bytes)
    {
        if (_renderer == null)
            throw new InvalidOperationException("Renderer not initialized");
        
        await _renderer.InvokeVoidAsync("handle_pan_camera_command", bytes);
    }

    public async Task HandleZoomCameraCommandAsync(byte[] bytes)
    {
        if (_renderer == null)
            throw new InvalidOperationException("Renderer not initialized");
        
        await _renderer.InvokeVoidAsync("handle_zoom_camera_command", bytes);
    }

    public async Task HandleResizeViewportCommandAsync(byte[] bytes)
    {
        if (_renderer == null)
            throw new InvalidOperationException("Renderer not initialized");
        
        await _renderer.InvokeVoidAsync("handle_resize_viewport_command", bytes);
    }

    public async Task RenderAsync()
    {
        if (_renderer == null)
            throw new InvalidOperationException("Renderer not initialized");
        
        await _renderer.InvokeVoidAsync("render");
    } 
}