
from shared.app.repos import AppUserTemplateMessageRepo
from shared.app.entities import AppUserTemplateMessageEntity


class AppUserTemplateMessageUsecase(Usecase):

    """ Usecase.
    This is doc string.
    """

    name = 'appusertemplatemessage'
    Entity = AppUserTemplateMessageEntity
    Repo = AppUserTemplateMessageRepo

    def create_appusertemplatemessage_record(self, **message):
        entity = self.repo.from_dict(message)
        entity = self.repo.create(entity)
        # comment
        return entity
