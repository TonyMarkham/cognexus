namespace CognexusBlazor.Services;

public interface ICameraService
{
    Task PanCameraAsync(float deltaX, float deltaY);
    Task ZoomCameraAsync(float scrollDelta, float pivotX, float pivotY);
}