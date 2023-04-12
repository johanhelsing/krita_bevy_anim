from krita import *

class KritaBevyExport(Extension):
    def __init__(self, parent):
        super().__init__(parent)
    
    def setup(self):
        print("Setup Bevy anim export")
        # appNotifier = Krita.instance().notifier()
        # appNotifier.setActive(True)
        # appNotifier.imageSaved.connect(self.handleImageSaved)

    # def handleImageSaved(self):
    #     self.exportDocument()

    def createActions(self, window):
        action = window.createAction("export_to_bevy", "Export for bevy", "tools/scripts")
        action.triggered.connect(self.exportDocument)

    def exportDocument(self):
        print("\nRendering pngs for bevy")
        # todo: actually configure the right settings?
        krita = Krita.instance()
        # it doesn't actually look like krita will let us decide where to
        # render the animation from within a script :(
        # so we'll make do with the previous location and let the user confirm
        # that they want to replace the old files...
        krita.action('render_animation_again').trigger()
        #    doc = Krita.instance().activeDocument()
        #    if doc is not None:
        #        fileName = QFileDialog.getSaveFileName()[0]
        #        doc.exportImage(fileName, InfoObject())

        # doc = krita.activeDocument()
        # directory = os.path.dirname(doc.fileName())
        # spriteName = os.path.splitext(os.path.basename(doc.fileName()))[0]

Krita.instance().addExtension(KritaBevyExport(Krita.instance()))